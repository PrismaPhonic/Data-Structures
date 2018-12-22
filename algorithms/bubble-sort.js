const bubbleSort = (arr) => {
  let hadToSort = true;
  let j = 0;
  while (hadToSort === true) {
    hadToSort = false;
    for (let i = 1; i < arr.length-j; i++) {
      if (arr[i-1] > arr[i]) {
        [arr[i], arr[i-1]] = [arr[i-1], arr[i]];
        hadToSort = true;
      }
    }
    j++;
  }
  return arr;
}

module.exports = bubbleSort;
