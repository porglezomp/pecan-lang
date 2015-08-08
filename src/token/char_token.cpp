#include "char_token.hpp"

#include <string>

std::string CharToken::show() {
  std::string show = "Char(";
  show += value;
  return show + ")";
}
