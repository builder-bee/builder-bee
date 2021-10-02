import { parse, Args } from "https://deno.land/std@0.109.0/flags/mod.ts";

function main(args: Args) {
	console.log(args);
}

main(parse(Deno.args));