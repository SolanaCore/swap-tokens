import { createCodamaConfig } from "gill";
 
export default createCodamaConfig({
  idl: "../idl/swap.json",
  clientJs: "../clients/js",
});