class Node {
  constructor(value, adjacent = new Set()) {
    this.value = value;
    this.adjacent = adjacent;
  }
}

class Graph {
  constructor() {
    this.nodes = new Set();
  }

  // this function accepts a Node instance and adds it to the nodes property on the graph
  addVertex(vertex) {
    this.nodes.add(vertex);
  }

  // this function accepts an array of Node instances and adds them to the nodes property on the graph
  addVertices(vertexArray) {
    vertexArray.forEach(vertex => {
      this.addVertex(vertex);
    });
  }

  // this function accepts two vertices and updates their adjacent values to include the other vertex
  addEdge(v1, v2) {
    v1.adjacent.add(v2);
    v2.adjacent.add(v1);
  }

  // this function accepts two vertices and updates their adjacent values to remove the other vertex
  removeEdge(v1, v2) {
    v1.adjacent.delete(v2);
    v2.adjacent.delete(v1);
  }

  // this function accepts a vertex and removes it from the nodes property, it also updates any adjacency lists that include that vertex
  removeVertex(vertex) {
    this.nodes.delete(vertex);
    this.nodes.forEach(node => {
      node.adjacent.delete(vertex);
    });
  }

  // this function returns an array of Node values using DFS
  // implemented using a recursive strategy - likely more efficient
  // as an iterative approach
  depthFirstSearch(start, visited = new Set()) {
    visited.add(start);

    for (let neighbor of start.adjacent) {
      if (!visited.has(neighbor)) {
        visited = this.depthFirstSearch(neighbor, visited);
      }
    }

    return visited;
  }

  // this function returns an array of Node values using BFS
  // iterative approach
  breadthFirstSearch(start) {
    const visited = new Set([start]);
    const queue = [start];

    while (queue.length > 0) {
      let current = queue.shift();
      for (let neighbor of current.adjacent) {
        if (!visited.has(neighbor)) {
          queue.push(neighbor);
          visited.add(neighbor);
        }
      }
    }

    return visited;
  }

  // this function returns the shortest path given an unweighted and
  // undirected graph
  shortestPath(start, end) {
    let queue = [[start]];
    let count = 0;
    let visited = new Set([start]);

    while (queue.length > 0) {
      let currGroup = queue.shift();

      let nextGroup = [];
      for (let current of currGroup) {
        if (current === end) return count;

        for (let neighbor of current.adjacent) {
          if (!visited.has(neighbor)) {
            nextGroup.push(neighbor);
            visited.add(neighbor);
          }
        }
      }
      if (nextGroup.length > 0) queue.push(nextGroup);
      count++;
    }
  }
}

module.exports = {Graph, Node};
