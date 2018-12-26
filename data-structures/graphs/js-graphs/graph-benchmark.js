const {Graph, Node} = require('./graph');
var Benchmark = require('benchmark');
var suite = new Benchmark.Suite();
let graph, a, b, c, d, S, P, U, X, Q, Y, V, R, W, T;

graph = new Graph();
a = new Node('A');
b = new Node('B');
c = new Node('C');
d = new Node('D');
S = new Node('S');
P = new Node('P');
U = new Node('U');
X = new Node('X');
Q = new Node('Q');
Y = new Node('Y');
V = new Node('V');
R = new Node('R');
W = new Node('W');
T = new Node('T');

graph.addVertices([S, P, U, X, Q, Y, V, R, W, T]);

graph.addEdge(S, P);
graph.addEdge(S, U);

graph.addEdge(P, X);
graph.addEdge(U, X);

graph.addEdge(P, Q);
graph.addEdge(U, V);

graph.addEdge(X, Q);
graph.addEdge(X, Y);
graph.addEdge(X, V);

graph.addEdge(Q, R);
graph.addEdge(Y, R);

graph.addEdge(Y, W);
graph.addEdge(V, W);

graph.addEdge(R, T);
graph.addEdge(W, T);

suite
  .add('shortestPath#test', function() {
    graph.shortestPath(P, W);
  })
  .on('complete', function() {
    var benchTest1 = this[0];

    console.log(benchTest1.times);
  })
  .run({async: true});
