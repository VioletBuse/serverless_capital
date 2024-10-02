const str_to_print: String = "hewwo everynyan! >.<";

Deno.core.print(str_to_print);
Deno.core.print(atob("haii"));

export default {
  event: async (trading) => {
    Deno.core.print("hewwo");

    return 5;
  },
  fetch: async (trading) => {
    return 8;
  },
};
