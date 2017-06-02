#include <ruby_parser/capi.hh>
#include <cstdio>

ruby_parser::typedruby24*
rbdriver_typedruby24_new(const char* source_ptr, size_t source_length, const ruby_parser::builder* builder)
{
	std::string source { source_ptr, source_length };
	return new ruby_parser::typedruby24(source, *builder);
}

void
rbdriver_typedruby24_free(ruby_parser::typedruby24* driver)
{
	delete driver;
}

const void*
rbdriver_parse(ruby_parser::base_driver* driver, ruby_parser::self_ptr self)
{
	return driver->parse(self);
}

bool
rbdriver_env_is_declared(const ruby_parser::base_driver *driver, const char* name, size_t length)
{
	std::string id { name, length };
	return driver->lex.is_declared(id);
}

void
rbdriver_env_declare(ruby_parser::base_driver *driver, const char* name, size_t length)
{
  std::string id { name, length };
  driver->lex.declare(id);
}

size_t
rbtoken_get_start(const ruby_parser::token* tok)
{
	return tok->start();
}

size_t
rbtoken_get_end(const ruby_parser::token* tok)
{
	return tok->end();
}

size_t
rbtoken_get_string(const ruby_parser::token* tok, const char** out_ptr)
{
	*out_ptr = tok->string().data();
	return tok->string().size();
}

size_t
rblist_get_length(const ruby_parser::node_list* list)
{
	return list->size();
}

const void*
rblist_index(ruby_parser::node_list* list, size_t index)
{
	return list->at(index);
}

size_t
rbdriver_diag_get_length(const ruby_parser::base_driver* driver)
{
	return driver->diagnostics.size();
}

ruby_parser::diagnostic_level
rbdriver_diag_get_level(const ruby_parser::base_driver* driver, size_t index)
{
	return driver->diagnostics.at(index).level();
}

size_t
rbdriver_diag_get_message(const ruby_parser::base_driver* driver, size_t index, const char** out_ptr)
{
	auto& message = driver->diagnostics.at(index).message();
	*out_ptr = message.data();
	return message.size();
}

size_t
rbdriver_diag_get_begin(const ruby_parser::base_driver* driver, size_t index)
{
	return driver->diagnostics.at(index).location().begin_pos;
}

size_t
rbdriver_diag_get_end(const ruby_parser::base_driver* driver, size_t index)
{
  return driver->diagnostics.at(index).location().end_pos;
}
