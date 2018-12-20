class Node {
  constructor(val, children = []) {
    this.val = val;
    this.children = children;
  }

  find(val) {
    let toVisitStack = [this];

    while (toVisitStack.length) {
      let current = toVisitStack.pop();

      if (current.val === val) return current;

      for (let child of current.children) toVisitStack.push(child);
    }
  }

  findBFS(val) {
    let toVisitQueue = [this];

    while (toVisitQueue.length) {
      let current = toVisitQueue.shift();

      if (current.val === val) return current;

      for (let child of current.children) toVisitQueue.push(child);
    }
  }
}

class Tree {
  constructor(root) {
    this.root = root;
  }

  findInTree(val) {
    return this.root.find(val);
  }

  findInTreeBFS(val) {
    return this.root.findBFS(val);
  }

  maxDepth(node = this.root, depth = 0, max = 0) {
    if (node.children.length === 0) return depth;

    for (let child of node.children) {
      let maxFromBranch = this.maxDepth(child, depth + 1, max);
      if (maxFromBranch > max) max = maxFromBranch;
    }

    return max;
  }

  minDepth() {
    // count depth until we reach the first leaf
    let toVisitQueue = [[this.root]];
    let depth = 0;

    loop1: while (toVisitQueue.length) {
      let currentRankArray = toVisitQueue.shift();
      let nextChildren = [];

      for (let neighbor of currentRankArray) {
        debugger;
        if (neighbor.children.length === 0) {
          break loop1;
        } else {
          nextChildren.push(...neighbor.children);
        }
      }

      if (nextChildren.length > 0) toVisitQueue.push(nextChildren);
      depth++;
    }

    return depth;
  }
}

// Keeping these functions separate from tree and node classes
// as they relate to only trees purely made of ints and don't
// serve as general purpose methods
function sumValues(tree) {
  let toVisitStack = [tree.root];
  let sum = 0;
  while (toVisitStack.length) {
    let current = toVisitStack.pop();

    sum += current.val;

    if (current.children.length > 0) {
      for (let child of current.children) {
        toVisitStack.push(child);
      }
    }
  }

  return sum;
}

function maxSum(node, max = 0, sum = 0) {
  sum += node.val;
  if (node.children.length === 0) return sum;

  debugger;
  for (let child of node.children) {
    let sumFromBranch = maxSum(child, max, sum);
    if (sumFromBranch > max) max = sumFromBranch;
  }

  return max;
}

function countEvens(tree) {
  let toVisitStack = [tree.root];
  let count = 0;
  while (toVisitStack.length) {
    let current = toVisitStack.pop();

    if (current.val % 2 === 0) count++;

    if (current.children.length > 0) {
      for (let child of current.children) {
        toVisitStack.push(child);
      }
    }
  }

  return count;
}

let intTree = new Tree(
  new Node(1, [
    new Node(2, [new Node(5), new Node(6)]),
    new Node(3, [new Node(7), new Node(8)]),
    new Node(4),
  ]),
);
