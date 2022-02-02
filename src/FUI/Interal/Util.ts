/**
 * 辅助类函数，此秘法不外传。
 */

export const selectClass = (classAttrs: Readonly<Record<string, boolean>>): string => {
	let xs = [];
	for (const k in classAttrs) {
		const b = classAttrs[k];
		if (b) {
			xs.push(k);
		}
	}

	return xs.join(" ");
}
