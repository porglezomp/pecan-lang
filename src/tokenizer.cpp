// Copyright 2015 Caleb Jones

#include "tokenizer.hpp"

#include <sstream>
#include <string>
#include <cctype>
#include <set>

#include "token/all_tokens.hpp"

std::set<char> operator_characters = {
  '+', '-', '*', '/', '=', '<', '>', '!', '%', '$',
  '?', ':', '#', '@', '&', '|', '^', '\\', '.'
};

void Tokenizer::advance_char()
{
  if (input.eof()) return;
  if (current_char == '\n') {
    line += 1;
    col = 0;
  }
  col += 1;
  current_char = input.get();
}

char Tokenizer::peek_char()
{
  return current_char;
}

void Tokenizer::advance()
{
  // Return if we're already at EOF
  if (current_token && current_token->type == EOFTOKEN) return;
  // Or if we just got to EOF
  if (input.eof()) {
    current_token = std::make_shared<EOFToken>(line, col);
    return;
  }
  
  // Skip over whitespace
  while (isspace(peek_char())) {
    advance_char();
    // If we hit EOF, then stop trying to advance
    if (input.eof()) {
      current_token = std::make_shared<EOFToken>(line, col);
      return;
    }
  }

  if (isalpha(peek_char()) || peek_char() == '_') {  // [_a-zA-Z][_a-zA-Z0-9]+
    std::string ident = "";
    while (isalnum(peek_char()) || peek_char() == '_') {
      ident += peek_char();
      advance_char();
    } 
    current_token = std::make_shared<IdentToken>(line, col, ident);
  } else if (isdigit(peek_char())) {  // [0-9]+ (. [0-9]*)?
    std::string num = "";
    while (isdigit(peek_char())) {
      num += peek_char();
      advance_char();
    }

    if (peek_char() == '.') {
      num += '.';
      advance_char();
      while (isdigit(peek_char())) {
	num += peek_char();
	advance_char();
      }
    }
    current_token = std::make_shared<NumToken>(line, col, num);
  } else {
    std::string op = "";
    while (operator_characters.find(peek_char()) != operator_characters.end()) {
      op += peek_char();
      advance_char();
    }

    if (op != "") {
      current_token = std::make_shared<OperatorToken>(line, col, op);
      if (op == "//") {
	while (peek_char() != '\n') advance_char();
	advance();
      }
    } else {
      current_token = std::make_shared<CharToken>(line, col, peek_char());
      advance_char();
    }
  }
}

std::shared_ptr<Token> Tokenizer::peek()
{
  return current_token;
}

