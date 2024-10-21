<script setup lang="ts">
import { onMounted } from "vue";
import { CheckMenuItem, IconMenuItem, Menu } from "@tauri-apps/api/menu";
import { transformImage } from "@tauri-apps/api/image";

onMounted(async () => {

  const menuItems = await Promise.all([
    IconMenuItem.new({
      text: "icon",
      id: "icon",
      icon: await transformImage("./icon.png"),
    }),
    CheckMenuItem.new({
      id: "alwaysOnTop",
      text: "alwaysOnTop",

      action: async () => {
        console.log("check item clicked");
      }
    }),
  ])

  const menu = await Menu.new({
    items: menuItems
  });
  await menu.setAsAppMenu();
})
</script>

<template>
  123
</template>