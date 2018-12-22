const bubbleSort = require('./bubble-sort');

describe('bubbleSort(arr)', () => {
  it('should correctly sort using bubble sort algo', () => {
    
    expect(bubbleSort([4, 20, 12, 10, 7, 9])).toEqual([4, 7, 9, 10, 12, 20]);
    expect(bubbleSort([0, -10, 7, 4])).toEqual([-10, 0, 4, 7]);
    expect(bubbleSort([1, 2, 3])).toEqual([1, 2, 3]);

    const nums = [4, 3, 5, 3, 43, 232, 4, 34, 232, 32, 4, 35, 34, 23, 2, 453, 546, 75, 67, 4342, 32];
    expect(bubbleSort(nums)).toEqual([2, 3, 3, 4, 4, 4, 5, 23, 32, 32, 34, 34, 35, 43, 67, 75, 232, 232, 453, 546, 4342]); 
  })
});
