// function test() {
//     return this.a;
// }

// var f = test.bind({a: "foo"});
// var b = f.bind({a: "bar"});
// var o = {a: 42, test: test, f: f, b: b};
// console.log(o.a, o.test(), o.f(), o.b());


var an_obj = {bar: function() {
                var x = (() => this);
                return x;
            }
        };

var fn = an_obj.bar();
console.log(fn());     // ???
var fn2 = an_obj.bar;
console.log(fn2()());  // ???