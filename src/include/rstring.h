#pragma once

#include <glib-object.h>

G_BEGIN_DECLS
G_DECLARE_FINAL_TYPE(ExRString, ex_rstring, Ex, RString, GObject)

ExRString* ex_rstring_new(const gchar* s);
ExRString* ex_rstring_copy(const ExRString* rstring);
void ex_rstring_free(ExRString* rstring);
gchar* ex_rstring_get(const ExRString* rstring);
void ex_rstring_set(ExRString* rstring,const gchar *s);
G_END_DECLS