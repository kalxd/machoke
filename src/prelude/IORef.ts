/**
 * 全局变量
 */
export default class IORef<T> {
	constructor(private value: T) {}

	write(value: T): T {
		this.value = value;
		return value;
	}

	read(): T {
		return this.value;
	}

	mapOut<R>(f: (ref: T) => R): R {
		return f(this.value);
	}
}
