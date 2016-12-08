var N = 500000;

function shuffle(a) {
  for (var i = a.length; i; i--) {
    var j = Math.floor(Math.random() * i);
    var x = a[j];
    var y = a[i - 1];
    a[j] = y;
    a[i - 1] = x;
  }
}

function main() {
  var nums = [];
  for (var i = 0; i < N; i++) {
    nums.push(i);
  }

  shuffle(nums);

  var start = Date.now();
  nums.sort();
  var end = Date.now();
  console.log('Sorted', N, 'ints in', end - start, 'ms');
}

main();
