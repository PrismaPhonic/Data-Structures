const { merge, mergeSort } = require('./merge-sort');

describe('merge(arr1, arr2)', () => {
  it('should correctly merge two pre-sorted arrays', () => {
    const arr1 = [1,3,4,5];
    const arr2 = [2,4,6,8];
    expect(merge(arr1,arr2)).toEqual([1,2,3,4,4,5,6,8])
  })
})

describe('mergeSort(arr)', () => {
  it('should correctly sort using mereg sort algo', () => {
    
    expect(mergeSort([4, 20, 12, 10, 7, 9])).toEqual([4, 7, 9, 10, 12, 20]);
    expect(mergeSort([0, -10, 7, 4])).toEqual([-10, 0, 4, 7]);
    expect(mergeSort([1, 2, 3])).toEqual([1, 2, 3]);

    const nums = [4, 3, 5, 3, 43, 232, 4, 34, 232, 32, 4, 35, 34, 23, 2, 453, 546, 75, 67, 4342, 32];
    expect(mergeSort(nums)).toEqual([2, 3, 3, 4, 4, 4, 5, 23, 32, 32, 34, 34, 35, 43, 67, 75, 232, 232, 453, 546, 4342]); 
  })
});
