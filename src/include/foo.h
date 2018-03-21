#pragma once

#include <glib-object.h>

G_BEGIN_DECLS
#define EX_TYPE_FOO (ex_foo_get_type())

G_DECLARE_FINAL_TYPE(ExFoo, ex_foo, EX, FOO, GObject)

ExFoo* ex_foo_new(const gchar* name);
gint ex_foo_increment(ExFoo* foo,gint inc);
gchar *ex_foo_get_name(ExFoo *foo);

G_END_DECLS