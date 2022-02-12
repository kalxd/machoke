/**
 * 预定义库
 */

import { Left, Right } from "purify-ts/Either";
import { EitherAsync } from "purify-ts/EitherAsync";

/**
 * 包装带有副作用的Promise，该Promise失败会抛出`Error`错误，之后转化成`EitherAsync<Error, T>`。
 *
 * @example
 * ```
 * const rsp = fromPromiseIO(() => fetch("/api"));
 * rsp.ifRight(response => console.log(response.text));
 * rsp.ifLeft(err => console.log(err));
 * ```
 *
 * @param f - 如何生成Promise
 */
export const fromPromiseIO = <T>(f: () => Promise<T>): EitherAsync<Error, T> =>
	EitherAsync.fromPromise(() =>
		f().then(Right).catch(Left));
