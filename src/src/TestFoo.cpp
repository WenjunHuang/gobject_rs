//
// Created by xxzyjy on 2018/3/10.
//
#include <iostream>
#include "catch.hpp"
#include "foo.h"

TEST_CASE("foo","[gobject_rs]"){
  auto foo = ex_foo_new("wenjun");
  std::cout << ex_foo_get_name(foo) << std::endl;
}

