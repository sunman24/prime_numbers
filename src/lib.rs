//noinspection ALL
/// Первый алгоритм проверки числа на простоту - наивный перебор.
/// В данном алгоритме мы просматриваем все числа от 2 до num (не включительно)
/// и проверяем делится ли оно на num. Сложность алгоритма O(n)
/// ## Аргументы:
/// ### num - проверяемое число
/// ## Возвращаемое значение:
/// ### Булево значение, которое означает является ли число простым
#[allow(dead_code)]
pub fn is_prime1(num: u64) -> bool {
    if num < 2 {
        false
    } else {
        (2..num + 1)
            .skip_while(|&i| num % i != 0)
            .next()
            .unwrap() == num
    }
}

//noinspection ALL
/// Второй алгоритм проверки числа на простоту - оптимизированный перебор.
/// Является усовершенствованием первого алгоритма.
/// Здесь мы уже проходим в цикле не до num, а до квадратного корня из num, округлённого
/// в меньшую сторону. Идея основного усовершенствования следующая: в исходном алгоритме не имеет
/// смысл просматривать абсолютно все числа до num, включая его, проверяя делимость num, на эти
/// числа. В самом деле, если число k является делителем num, то и число num / k также является
/// делителем num. Поэтому проход в цикле идёт до квадратного корня из num.
/// ## Аргументы:
/// ### num - проверяемое число
/// ## Возвращаемое значение:
/// ### Булево значение, которое означает является ли число простым
#[allow(dead_code)]
pub fn is_prime2(num: u64) -> bool {
    let fin = (num as f64).sqrt().floor() as u64;

    if num < 2 {
        false
    } else {
        (2..fin + 1).skip_while(|&i| num % i != 0).next().is_none()
    }
}

//noinspection ALL
/// Третий алгоритм проверки числа на простоту - оптимизированный перебор с использованием цикла.
/// Является аналогом второго алгоритма за тем лишь исключением, что вместо использования
/// итераторов и функций высшего порядка, определённых для него, используется простой цикл.
/// ## Аргументы:
/// ### num - проверяемое число
/// ## Возвращаемое значение:
/// ### Булево значение, которое означает является ли число простым
#[allow(dead_code)]
pub fn is_prime3(num: u64) -> bool {
    let fin = (num as f64).sqrt().floor() as u64;

    return if num < 2 {
        false
    } else {
        for i in 2..(fin + 1) {
            if num % i == 0 {
                return false;
            }
        }

        return true;
    }
}

//noinspection ALL
#[allow(unused)]
pub fn is_prime4(num: u64) -> bool {
    let fin = (num as f64).sqrt().floor() as u64;

    if num < 2 {
        return false;
    } else {
        for i in 2..fin + 1 {
            if num % i == 0 {
                return false;
            }
        }

        return true;
    }
}
