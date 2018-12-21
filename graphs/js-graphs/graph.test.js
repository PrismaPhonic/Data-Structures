const {Graph, Node} = require('./graph');
let graph, a, b, c, d, S, P, U, X, Q, Y, V, R, W, T;

//insert test graph data before each test
beforeEach(function() {
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
});

describe('addVertex/addVertices', () => {
  it('should correctly add vertices to Graph instance', () => {
    graph.addVertices([a, b]);
    graph.addVertex(c);

    expect(graph.nodes.has(a)).toBeTruthy();
    expect(graph.nodes.has(b)).toBeTruthy();
    expect(graph.nodes.has(c)).toBeTruthy();
  });
});

describe('graph.addEdge()', () => {
  it('should add edges and end up in correct adjacency lists', () => {
    graph.addVertices([a, b, c, d]);
    graph.addEdge(a, b);
    graph.addEdge(a, c);
    graph.addEdge(b, d);
    graph.addEdge(c, d);

    expect(a.adjacent).toEqual(new Set([b, c]));
    expect(b.adjacent).toEqual(new Set([a, d]));
    expect(c.adjacent).toEqual(new Set([a, d]));
    expect(d.adjacent).toEqual(new Set([b, c]));
  });
});


describe('graph.removeEdge()', () => {
  it('should remove edge specified and delete nodes from both adjacency lists', () => {
    graph.addVertices([a, b, c, d]);
    graph.addEdge(a, b);
    graph.addEdge(a, c);
    graph.addEdge(b, d);
    graph.addEdge(c, d);

    graph.removeEdge(b,a);
    graph.removeEdge(c,d);

    expect(a.adjacent.has(b)).toBeFalsy();
    expect(b.adjacent.has(a)).toBeFalsy();
    expect(c.adjacent.has(d)).toBeFalsy();
    expect(d.adjacent.has(c)).toBeFalsy();
  });
});

describe('graph.removeVertex()', () => {
  it('should remove vertex specified and delete node from all adjacency lists', () => {
    graph.addVertices([a, b, c, d]);
    graph.addEdge(a, b);
    graph.addEdge(a, c);
    graph.addEdge(b, d);
    graph.addEdge(c, d);

    graph.removeVertex(c);
    graph.removeVertex(d);

    expect(graph.nodes.has(a)).toBeTruthy();
    expect(graph.nodes.has(b)).toBeTruthy();
    expect(graph.nodes.has(c)).toBeFalsy();
    expect(graph.nodes.has(d)).toBeFalsy();
  });
});

describe('graph.depthFirstSearch()', () => {
  it('should perform an exhaustive depth first search', () => {
    graph.addVertices([S,P,U,X,Q,Y,V,R,W,T])

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

    expect(graph.depthFirstSearch(S)).toEqual(new Set([S, P, U, X, Q, V, Y, R, W, T]))
  });
});

describe('graph.breadthFirstSearch()', () => {
  it('should perform an exhaustive breadth first search', () => {
    graph.addVertices([S,P,U,X,Q,Y,V,R,W,T])

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

    expect(graph.breadthFirstSearch(S)).toEqual(new Set([S, U, V, W, T, R, Q, Y, X, P]))
  });
});

describe('graph.shortestPath()', () => {
  it('should return the shortest path between two points', () => {
    graph.addVertices([S,P,U,X,Q,Y,V,R,W,T])

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

    expect(graph.shortestPath(P, V)).toEqual(2);
    expect(graph.shortestPath(P, Y)).toEqual(2);
    expect(graph.shortestPath(P, W)).toEqual(3);
    expect(graph.shortestPath(S, T)).toEqual(4);
  });
});
