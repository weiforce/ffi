var ffi = require("ffi-napi")

var libgetquery = ffi.Library('./rustffi/target/debug/librustffi.so', {
  get_query: ['string', ['string']]
})

if (process.argv.length < 3) {
  console.log('Arguments: ' + process.argv[0] + ' ' + process.argv[1] + ' <max>')
  process.exit()
}

var output = libgetquery.get_query(process.argv[2])

console.log('Your output: ' + output)
