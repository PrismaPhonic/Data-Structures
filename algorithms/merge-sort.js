const merge = (arr1, arr2) => {
  let p1 = 0, p2 = 0;
  let merged = [];
  while (p1 < arr1.length || p2 < arr2.length) {
    if (p2 >= arr2.length || arr1[p1] <= arr2[p2]) {
      merged.push(arr1[p1]);
      p1++;
    } else if (p1 >= arr1.length || arr2[p2] <= arr1[p1]) {
      merged.push(arr2[p2]);
      p2++
    }
  }

  return merged;
}

const mergeSort = (arr) => {
  // let's use recursion to break down the array into arrays of 1 or none
  if (arr.length <= 1) {
    return arr;
  } else {
    let arr1 = arr.slice(0,Math.floor(arr.length/2));
    let arr2 = arr.slice(Math.floor(arr.length/2));
    let s1 = mergeSort(arr1);
    let s2 = mergeSort(arr2);
    return merge(s1, s2);
  }
}

module.exports = { 
  merge,
  mergeSort
};
