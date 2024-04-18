export class List {
    constructor(generator, length = Infinity) {
        this[Symbol.iterator] = generator
        this.length = length
    }

    static get integers () {
        return this.range(0, Infinity)
    }

    static get fibonacci () {
        return new List(function* () {
            let x = 1
            let y = 1
            yield* [ 0, x, y ]

            while (true) {
                let next = x + y
                yield next
                x = y
                y = next
            }
        }, Infinity)
    }

    static of (...args) {
        return new List(function* () {
            yield* args
        }, args.length)
    }

    static from (iterable) {
        return new List(function* () {
            yield* iterable
        }, iterable.length)
    }

    static range (start, end, step = 1) {
        return new List(function* () {
            let i = start
            while (i <= end) {
                yield i
                i += step
            }
        }, Math.floor((end - start + 1) / step))
    }

    static empty () {
        return new List(function* () {}, 0)
    }

    concat (iterable) {
        const generator = this[Symbol.iterator]
        return new List(function* () {
            yield* generator()
            yield* iterable
        }, this.length + iterable.length)
    }

    map (mapper) {
        const generator = this[Symbol.iterator]
        return new List(function* () {
            for (const value of generator()) {
                yield mapper(value)
            }
        }, this.length)
    }

    filter (predicate) {
        const generator = this[Symbol.iterator]
        return new List(function* () {
            for (const value of generator()) {
                if (predicate(value)) yield value
            }
        }, this.length)
    }

    scan (scanner, seed) {
        const generator = this[Symbol.iterator]
        return new List(function* () {
            let acc = seed
            for (const value of generator()) {
                yield acc = scanner(acc, value)
            }
        }, this.length)
    }

    reduce (reducer, seed) {
        return this.toArray().reduce(reducer, seed)
    }

    ap (list) {
        const generator = this[Symbol.iterator]
        return new List(function* () {
            for (const f of generator()) {
                yield* list.map(f)
            }
        }, this.length)
    }

    take (x) {
        const generator = this[Symbol.iterator]
        return new List(function* () {
            const iterator = generator()
            let next = iterator.next()
            let n = 0

            while (!next.done && x > n) {
                yield next.value
                n++
                next = iterator.next()
            }
        }, this.length > x ? x : this.length)
    }

    drop (x) {
        const generator = this[Symbol.iterator]
        return new List(function* () {
            const iterator = generator()
            let next = iterator.next()
            let n = 1

            while (!next.done) {
                if (n > x) yield next.value
                n++
                next = iterator.next()
            }
        }, this.length - x)
    }

    zipWith (lazyList, zipper) {
        const generator1 = this[Symbol.iterator]
        const generator2 = lazyList[Symbol.iterator]
        return new List(function* () {
            const iterator1 = generator1()
            const iterator2 = generator2()
            let next1 = iterator1.next()
            let next2 = iterator2.next()
            let i = 0

            while (!next1.done && !next2.done) {
                yield zipper(next1.value, next2.value)
                next1 = iterator1.next()
                next2 = iterator2.next()
            }
        }, this.length < lazyList.length ? this.length : lazyList.length)
    }

    head () {
        return this[Symbol.iterator]().next().value
    }

    tail () {
        return this.drop(1)
    }

    toArray () {
        return [ ...this ]
    }

    toString () {
        const displayedCount = 100
        return `List [ ${
            this.take(displayedCount).toArray().join(', ')
        }${
            this.length > displayedCount ? ' ...' : ''
        } ]`
    }

    inspect () {
        return this.toString()
    }
}
