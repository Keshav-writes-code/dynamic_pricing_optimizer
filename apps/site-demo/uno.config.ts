import { defineConfig } from "unocss";
import { presetDaisyui } from "@0x-jerry/unocss-preset-daisyui";
import presetIcons from "@unocss/preset-icons";
import presetMini from "@unocss/preset-mini";

export default defineConfig({
  presets: [presetMini(), presetDaisyui(), presetIcons()],
  // ...UnoCSS options
});
