/**
 * 全局变量
 */
export default class IORef<T> {
	constructor(private value: T) {}

	write(value: T): this {
		this.value = value;
		return this;
	}

	writeWith<K extends keyof T>(key: K, value: T[K]): this {
		this.value[key] = value;
		return this;
	}

	read(): T {
		return this.value;
	}

	readWith<K extends keyof T>(key: K): T[K] {
		return this.value[key];
	}

	mapOut<R>(f: (ref: T) => R): R {
		return f(this.value);
	}
}
