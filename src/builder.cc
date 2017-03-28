#include <ruby_parser/builder.hh>

using namespace ruby_parser;

node* builder::compstmt(std::unique_ptr<list_node> node)
{
    if (node->nodes.size() == 0) {
        return nullptr;
    } elsif (node->nodes.size() == 1) {
        return node->nodes[0].release();
    } else {
        return new begin_node(node->nodes);
    }
}
