import type { Config } from "tailwindcss";

function generateGridTemplateRows(nums: number[]): Record<number, string> {
    let result: Record<number, string> = {};
    for (const num of nums) {
        if (result[num] === undefined) {
            result[num] = `repeat(${num}, minmax(0, 1fr))`;
        }
    }
    return result;
}

const config = {
    content: ["./src/**/*.rs"],
    theme: {
        extend: {
            gridTemplateRows: generateGridTemplateRows([16])
        }
    },
    plugins: []
} satisfies Config;

export default config;
