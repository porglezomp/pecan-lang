// Copyright 2015 Caleb Jones

#include "tokenizer.hpp"

#include <sstream>
#include <string>
#include <cctype>

#include "token/all_tokens.hpp"

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
  while (isspace(peek_char())) {
    advance_char();
    if (input.eof()) {
      current_token = std::make_shared<Token>(EOFToken(line, col));
      return;
    }
  }
  if (isalpha(peek_char()) || peek_char() == '_') {
    std::string ident = "";
    while (isalnum(peek_char()) || peek_char() == '_') {
      ident += peek_char();
      advance_char();
    } 
    current_token = std::make_shared<Token>(IdentToken(line, col, ident));
  } else if (isdigit(peek_char())) {
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
    current_token = std::make_shared<Token>(NumToken(line, col, num));
  } else {
    current_token = std::make_shared<Token>(CharToken(line, col, peek_char()));
  }
}

std::shared_ptr<Token> Tokenizer::peek()
{
  return current_token;
}

