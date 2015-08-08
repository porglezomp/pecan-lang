#ifndef PECAN_TOKEN_NUM_TOKEN_HPP_INCLUDED
#define PECAN_TOKEN_NUM_TOKEN_HPP_INCLUDED

#include "token.hpp"

#include <cstdint>

class NumToken : public Token {
  std::string value;
public:
  double doubleValue();
  int64_t signedValue();
  uint64_t unsignedValue();
  virtual std::string show();
  NumToken(int line, int col, std::string value)
    : Token(line, col, NUMERIC), value(value) { }

  virtual ~NumToken() { }
};

#endif
