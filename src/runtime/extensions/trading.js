const { core } = Deno;

const str = "hewwo >.< from trading api \n";

core.ops.exchange().then((str) => {
  Deno.core.print(`exchange: ${str}\n`);
});

Deno.core.print(str);
