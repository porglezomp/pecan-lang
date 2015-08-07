#ifndef PECAN_TOKENIZER_HPP_INCLUDED
#define PECAN_TOKENIZER_HPP_INCLUDED

#include <iostream>
#include <memory>

#include "token/all_tokens.hpp"

class Tokenizer {
  std::istream &input;
  int line = 0, col = 0;
  char current_char = '\n';
  std::shared_ptr<Token> current_token;

  void advance_char();
  char peek_char();

public:
  void advance();
  std::shared_ptr<Token> peek();

  Tokenizer(std::istream &in) : input(in) {
    advance_char();
    advance();
  }
};

#endif
