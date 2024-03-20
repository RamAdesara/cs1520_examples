// const a = new Array(15, 20, 5, 7, 100, 80, 99);
// a.sort();
// console.log(a);

// const a = new Array(1, 2, 3, 4, 5);
// const b = new Array(a[5], a[6], a[7]);
// console.log(a);
// console.log(b);

// const a = new Array(1, 2, 3, 4, 5);
// // a.length = 7;
// const s = a.join("!");
// console.log(s);

const f = (x) => x * 10;
let x = f(5);


// this means that x contains 5 * 10 = 50.

const g = () => x + 5;
// this means that g contains the function, that when called, returns 50 + 5, or 55.

x = g();
// this means that x contains 55.
console.log(x);