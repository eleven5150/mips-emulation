const process = require('process');
const fs = require('fs');

let DATA_TO_SORT_SIZE = 0;

function data_print(data, size) {
    for (let i = 0; i < size; i++) {
        console.log(`${data[i]} `);
    }
}

function partition(data, low, high) {
    let temp;
    let pivot = data[high];
    let i = low - 1;
    for (let j = low; j <= high - 1; j++) {
        if (data[j] <= pivot) {
            i++;
            temp = data[i];
            data[i] = data[j];
            data[j] = temp;
        }
    }

    temp = data[i + 1];
    data[i + 1] = data[high];
    data[high] = temp;

    return i + 1;
}

function quick_sort(items, low, high) {
    if (low < high) {
        let pi = partition(items, low, high);

        quick_sort(items, low, pi - 1);
        quick_sort(items, pi, high);
    }
    return items;
}

function main(argc, argv) {
    if (argc < 1) {
        console.error("Error! File with data to sort must be specified\n");
        return 1;
    }

    let data_to_sort_path = argv[2];

    let data = fs.readFileSync(data_to_sort_path, "utf8");
    if (!data) {
        console.error(`File ${data_to_sort_path} opening failed`);
        return 0;
    }

    let strings = data.split(/\n/);
    strings.pop();
    let data_to_sort_size = strings[0];
    DATA_TO_SORT_SIZE = data_to_sort_size.slice(2);
    strings = strings.slice(1);

    let data_to_sort = new BigUint64Array(DATA_TO_SORT_SIZE);
    for (let line_num in strings) {
        data_to_sort[line_num] = BigInt(parseInt(strings[line_num], 10));
    }

    quick_sort(data_to_sort, 0, DATA_TO_SORT_SIZE - 1);

    // data_print(data_to_sort, DATA_TO_SORT_SIZE);

}

main(process.argv.length, process.argv);