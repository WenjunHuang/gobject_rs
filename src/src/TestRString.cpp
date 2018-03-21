//
// Created by xxzyjy on 2018/3/19.
//

#include <memory>
#include "catch.hpp"
#include "rstring.h"

struct ExRStringDeleter {
  void operator()(ExRString *rstring) {
    ex_rstring_free(rstring);
  }
};

TEST_CASE("RString", "[ffi]") {
  using ExRStringPtr = std::unique_ptr<ExRString, ExRStringDeleter>;

  ExRStringPtr rstring(ex_rstring_new("hello rust"));

  REQUIRE(strcmp("hello rust", ex_rstring_get(rstring.get())) == 0);

  ex_rstring_set(rstring.get(), "hello again rust");
  REQUIRE(strcmp("hello again rust", ex_rstring_get(rstring.get())) == 0);

  ExRStringPtr new_rstring(ex_rstring_copy(rstring.get()));
  ex_rstring_set(rstring.get(), "hello again again rust");
  REQUIRE(strcmp("hello again rust", ex_rstring_get(new_rstring.get())) == 0);
  REQUIRE(strcmp("hello again again rust", ex_rstring_get(rstring.get())) == 0);
}