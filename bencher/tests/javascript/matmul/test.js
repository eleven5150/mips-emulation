
function multiply(mat1, mat2, res)
{
    let i, j, k;
    let N = mat1.length
    for (i = 0; i < N; i++) {
        for (j = 0; j < N; j++) {
            res[i][j] = 0;
            for (k = 0; k < N; k++)
            res[i][j] += mat1[i][k] * mat2[k][j]
        }
    }
}

let fs = require('fs');
let all = fs.readFileSync('tests/datasets/matmul/dataset.txt', "utf8");
all = all.trim();  // final crlf in file
let lines = all.split("\n");
let n = lines.length;
var mas = [];
	for (var i = 0; i < n; i++){
	    mas[i] = [];
	    for (var j = 0; j < n; j++){
	        mas[i][j] = 0;
	}}

for (let i = 0; i < n; ++i) {  // each line
  let tokens = lines[i].split(" ");
  for (let j = 0; j < n; ++j) {  // each val curr line
    mas[i][j] = parseInt(tokens[j]);
  }
}

var res = [];
	for (var i = 0; i < n; i++){
	    res[i] = [];
	    for (var j = 0; j < n; j++){
	        res[i][j] = 0;
	}}

multiply(mas,mas,res)
