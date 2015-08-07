#ifndef PECAN_TOKEN_TOKEN_HPP_INCLUDED
#define PECAN_TOKEN_TOKEN_HPP_INCLUDED

enum TokenType {
  INVALID,
  IDENTIFIER,
  OPERATOR,
  KEYWORD,
  CHARACTER,
  NUMERIC,
  EOFTOKEN,
};

class Token {
  const int line, col;
public:
  const TokenType type = INVALID;
  Token(int line, int col) : line(line), col(col) { }
  virtual ~Token() { }
};

#endif
