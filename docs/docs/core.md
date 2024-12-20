# Core Library

EasyJS includes a optional core library that can be invocked with the `use_mod` builtin.

## builtins.ej
Includes helpful builtin functions for ease of use.

### int_range
This method is used to create a range from var..var in easyjs.
```easyjs
export fn int_range(start, end) {
    var res = []
    // the .. works because this is a for loop...
    for i in start..end {
        res.push(i)
    }

    return res
}
```

### sleep
```ej
export fn sleep(ms) {
    javascript{
        return new Promise(resolve => setTimeout(resolve, ms))
    }
}
```

### range
```ej
/// Creates a range
export fn range(kwargs) {
    start = kwargs.start
    end = kwargs.end
    step = kwargs.step ?? 1

    javascript{
        return Array(Math.ceil((end - start) / step)).fill(start).map((x,y) => x + y * step)
    }
}
```

### flat
```ej
/// Flatten a list in JS.
export fn flat(list) {
    javascript{
        return [...new Set(list)];
    }
}
```

### capitalize
```ej
/// Capitalize a string
export fn capitalize(str) {
    return "${str.charAt(0).toUpperCase()}${str.slice(1)}"
}
```

### merge
```ej
/// Merge 2 arrays
export fn merge(arr1, arr2, flatten) {
    var narr = [].concat(arr1, arr2)

    if flatten == true {
        return flat(narr)
    }

    return arr
}
```

### reverse_string
```ej
/// Reverse a string
export reverse_string = fn(str) {return str.split("").reverse().join("")} 
```

### easyjs_ascii
```ej
/// Get the EasyJS ASCII
export fn easyjs_ascii() {
    return "    ___       ___       ___       ___            ___       ___   
   /\\  \\     /\\  \\     /\\  \\     /\\__\\          /\\  \\     /\\  \\  
  /::\\  \\   /::\\  \\   /::\\  \\   |::L__L        _\\:\\  \\   /::\\  \\ 
 /::\\:\\__\\ /::\\:\\__\\ /\\:\\:\\__\\  |:::\\__\\      /\\/::\\__\\ /\\:\\:\\__\\
 \\:\\:\\/  / \\/\\::/  / \\:\\:\\/__/  /:;;/__/      \\::/\\/__/ \\:\\:\\/__/
  \\:\\/  /    /:/  /   \\::/  /   \\/__/          \\/__/     \\::/  / 
   \\/__/     \\/__/     \\/__/                              \\/__/  "
}
```

## date.ej
In JS it is known that dates are hard to work with. The date module is ment to make it easier working with dates.

### days_between_dates
```ej
/// Get the days between 2 dates
export days_between_dates = fn(d1, d2) { return Math.ceil(Math.abs(date1 - date2) / (1000 * 60 * 60 * 24)) }
```

### get_week_day
```ej
/// Get the weekday of a date.
export get_week_day = fn(d) { return d.toLocaleString('en-US', {weekday: 'long'}) }
```

### is_weekend
```ej
/// Is a date a weekend?
export is_weekend = fn(d) {return [5,6].indexOf(d.getDay()) != -1}
```