/**
 * 网络请求相关
 */

export type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export const httpMethods: Readonly<Array<HttpMethod>> = [
	"GET",
	"POST",
	"PUT",
	"PATCH",
	"DELETE"
];
