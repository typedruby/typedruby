#ifndef RUBY_PARSER_NODE_HH
#define RUBY_PARSER_NODE_HH

namespace ruby_parser {
    enum class NodeType {

    };

    class Node {
    public:
        virtual NodeType type() const = 0;
    };
}

#endif
