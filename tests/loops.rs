#[cfg(test)]
mod loops {
    #[test]
    fn forloop() {
        let mut sum = 0;
        for i in 1..4 {
            sum += i
        }
        assert_eq!(sum, 6);

        let a = [1, 2, 3];
        let iter = a.iter();
        sum = 0;
        for i in iter {
            sum += i
        }
        assert_eq!(sum, 6);
    }

    #[test]
    fn whileloop() {
        let mut sum = 0;
        while sum < 6 {
            sum += 1
        }
        assert_eq!(sum, 6);
    }

    #[test]
    fn loopbreak() {
        let mut sum = 0;
        loop {
            sum += 1;
            if sum == 6 {
                break;
            }
        }
        assert_eq!(sum, 6);
    }

    #[test]
    fn forcontinue() {
        let mut sum = 0;
        for i in 1..4 {
            if i % 2 == 0 {
                continue;
            }
            sum += i;
        }
        assert_eq!(sum, 4);
    }
}