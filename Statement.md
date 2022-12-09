```rs
STATEMENT => ce qui apparait
{
    functions : HashMap of FuncNode, (String /*for the name as a key*/)
    // => When the program start, the interpreter gonna read the "main" function.
    classes: HashMap of ClassNode, (String /*for the name as a key*/)
    structs: HashMap of StructNode, (String /*for the name as a key*/)
    variables: HashMap of VarAccessNode (String /*for the name as a key*/)
    ...
}

// Exemple of a STATEMENT


class Hello {
    func printHelloWorld() {
        print "Hello World";
    }
}

const A = 5;

func main() {
    if A > 5 {
        println "WTF?";
    } elif A < 5 {
        println "WTF2?";
    } else {
        println "Yeah, like that";
    }
}

struct Statement {
    functions: HashMap("main", FuncNode {
        name: "main",
        args: None,
        return: None,
        body: Vector(IfNode {
            ifCase: IfCase {
                comparison: CompNode {
                    left: A,
                    operator: ">",
                    right: 5
                },
                body: Vector(PrintNode {
                    ln: true,
                    contain: "WTF?"
                })
            },
            elifCases: Vector(IfNode {
                comparison: CompNode {
                    left: A,
                    operator: "<",
                    right 5
                },
                body: Vector(PrintNode {
                    ln: true,
                    contain: "WTF2?"
                })
            }),
            elseCase: ElseNode {
                body: Vector<PrintNode {
                    ln: true,
                    contain: "Yeah, like that"
                })
            }
        })
    }),
    classes: HashMap("Hello", ClassNode {
        constructor: None,
        attributes: Vector(),
        body: Vector(FuncNode {
            name: "printHelloWorld",
            args: None,
            return: None,
            body: Vector(PrintNode {
                ln: false,
                contain: "Hello World"
            })
        }
    })
}
```