const str_to_print: String = "hewwo everynyan! >.<\n";

Deno.core.print(str_to_print);
Deno.core.print(atob("haii\n"));

export default {
  event: async (trading) => {
    Deno.core.print("hewwo\n");

    Deno.core.print(`tenant id: ${tenant_id} \n`);

    return 5;
  },
  fetch: async (trading) => {
    return 8;
  },
  signal: async () => {},
  scheduled: async () => {},
};
