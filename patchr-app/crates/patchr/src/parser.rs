use std::{any::type_name_of_val, collections::LinkedList, ops::ControlFlow};

use log::{debug, trace};

use crate::{
    cli_print, cli_print_error, commands::{
        get_command_builder, help::Help, set_verbose::SetVerbose, Command, CommandBuilder,
        CommandBuilderError, HELP, VERBOSE,
    }
};

pub fn parse_command_line(command_line: Vec<String>) -> Option<Vec<Box<dyn Command>>> {
    let lexer = Lexer::new(command_line);
    let parser = Parser::new();
    parser.parse_tokens(lexer)
}

struct Lexer {
    preview: LinkedList<Token>,
    it: Box<dyn Iterator<Item = String>>,
}

struct Parser;
struct ParserState {
    last_flag: Option<Token>,
    main_command: Option<Box<dyn CommandBuilder>>,
    result: Vec<Box<dyn Command>>,
}

#[derive(Debug, Clone)]
enum TokenType {
    Flag,
    Value,
}

#[derive(Debug, Clone)]
struct Token {
    ttype: TokenType,
    str: String,
}

impl Token {
    fn new(ttype: TokenType, str: String) -> Self {
        Self { ttype, str }
    }
}

struct ParsingError {
    message: String,
    token: Token,
}

impl ParsingError {
    fn new(message: String, token: &Token) -> Self {
        Self {
            message,
            token: token.clone(),
        }
    }

    fn from_cmd_builder_err(err: CommandBuilderError, token: &Token) -> Self {
        Self::new(format!("{:?} {}", err.code(), err.message_move()), token)
    }
}

impl Lexer {
    fn new(data: Vec<String>) -> Self {
        let mut it = Box::new(data.into_iter());
        it.next(); // skip the binary name
        Self {
            preview: LinkedList::new(),
            it,
        }
    }

    fn update_preview(&mut self, c: &String) {
        c[1..].chars().for_each(|f| {
            self.preview
                .push_back(Token::new(TokenType::Flag, String::from(f)));
        });
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.preview.is_empty() {
            return self.preview.pop_front();
        }

        let Some(c) = self.it.next() else {
            return None;
        };
        if c.starts_with("-") {
            self.update_preview(&c);
            self.preview.pop_front()
        } else {
            Some(Token::new(TokenType::Value, c))
        }
    }
}

impl ParserState {
    fn new() -> Self {
        Self {
            last_flag: None,
            main_command: Option::<Box<dyn CommandBuilder>>::None,
            result: Vec::new(),
        }
    }
}

impl Parser {
    fn new() -> Self {
        Self {}
    }

    fn parse_global_option(&self, token: &Token) -> Result<Box<dyn Command>, ParsingError> {
        match token.str.as_str() {
            VERBOSE => Ok(Box::new(SetVerbose::new())),
            HELP => Ok(Box::new(Help::new())),
            _ => Err(ParsingError::new(
                String::from(format!("Parsing failed due to unknown option",)),
                token,
            )),
        }
    }

    fn parse_flag(
        &self, state: &mut ParserState, token: &Token,
    ) -> Result<Option<Box<dyn Command>>, ParsingError> {
        if let Some(last_flag) = state.last_flag.as_ref() {
            return Err(ParsingError::new(
                format!("Expected a value for flag '{}'", last_flag.str),
                token,
            ));
        };
        if let Some(main_command) = &mut state.main_command {
            // if there is a main command, check if it accepts the flag
            if let Ok(requires_value) = main_command.requires_value(token.str.as_str()) {
                if requires_value {
                    state.last_flag = Some(token.clone());
                } else {
                    // flag is acceptes but does not require a value
                    main_command.add_flag(token.str.as_str()).unwrap();
                }
                return Ok(None);
            }
        }
        self.parse_global_option(token).map(|r| Some(r))
    }

    fn parse_value(
        &self, state: &mut ParserState, token: &Token,
    ) -> Result<Option<Box<dyn Command>>, ParsingError> {
        let result = if let Some(main_command) = &mut state.main_command {
            let builder_res = if let Some(last_flag) = &state.last_flag {
                main_command.add_flag_and_value(last_flag.str.as_str(), token.str.as_str())
            } else {
                main_command.add_value(token.str.as_str())
            };
            builder_res
                .map(|_| None)
                .map_err(|e| ParsingError::from_cmd_builder_err(e, token))
        } else {
            if let Some(_last_flag) = &state.last_flag {
                // TODO : add support for global flags with value
                Err(ParsingError::new(
                    String::from("Global options with values not supported yet"),
                    token,
                ))
            } else {
                trace!("trying to get the main command : {}", token.str);
                state.main_command = get_command_builder(token.str.as_str());
                Ok(None)
            }
        };
        result
    }

    fn process_token(
        &self, state: &mut ParserState, token: &Token,
    ) -> Result<Option<Box<dyn Command>>, ParsingError> {
        trace!("processing {:?} : {}", token.ttype, token.str);
        match token.ttype {
            TokenType::Flag => self.parse_flag(state, token),
            TokenType::Value => self.parse_value(state, token),
        }
    }

    fn parse_tokens(&self, mut lexer: Lexer) -> Option<Vec<Box<dyn Command>>> {
        let result = lexer.try_fold(ParserState::new(), |mut state, token| {
            match self.process_token(&mut state, &token) {
                Ok(command_opt) => {
                    if let Some(command) = command_opt {
                        state.result.push(command);
                    }
                    ControlFlow::Continue(state)
                }
                Err(e) => ControlFlow::Break(e),
            }
        });
        match result {
            ControlFlow::Continue(mut r) => {
                let Some(builder) = r.main_command else {
                    cli_print!("No main command");
                    return Some(r.result)
                };

                match builder.build() {
                    Ok(c) => {
                        trace!("Main command found: {}", type_name_of_val(&c));
                        r.result.push(c);
                        Some(r.result)
                    }
                    Err(e) => {
                        cli_print_error!("Cannot execute command {}", builder.name());
                        cli_print_error!("{}", e.message());
                        return None;
                    }
                }
            }
            ControlFlow::Break(e) => {
                cli_print_error!("{}", e.message);
                debug!("Invalid token : {:?}", e.token);
                None
            }
        }
    }
}
