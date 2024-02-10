'use strict';

const process = require('process');
const fs = require('fs');

function main(argc, argv) {
    if (argc < 1) {
        console.error("Error! File with prime number count must be specified\n");
        return 1;
    }

    let prime_number_count_file = argv[2];

    let data = fs.readFileSync(prime_number_count_file, "utf8");
    if (!data) {
        console.error(`File ${prime_number_count_file} opening failed`);
        return 0;
    }

    let prime_number_count = BigInt(parseInt(data));

    let curr_number = 0n;
    while (prime_number_count > 0) {
        curr_number++;
        let j = 0n;

        for (let i = 1n; i <= curr_number; i++) {
            if (curr_number % i === 0n) {
                j++;
            }
        }

        if (j === 2n) {
            prime_number_count--;
        }
    }

    console.log(`The latest prime number: ${curr_number}`);
}

main(process.argv.length, process.argv);
