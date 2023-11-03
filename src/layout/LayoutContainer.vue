<template>
  <div class="layout">
    <TinyContainer>
      <template #header>
        <LayoutHeader></LayoutHeader>
      </template>
      <template #aside>
        <LayoutAside></LayoutAside>
      </template>
      <LayoutContent></LayoutContent>
      <template #footer>
        <LayoutFooter></LayoutFooter>
      </template>
    </TinyContainer>
  </div>
</template>

<script>
import {defineComponent} from "vue";
import {Container as TinyContainer} from '@opentiny/vue'
import LayoutHeader from "./compoments/LayoutHeader.vue";
import LayoutAside from "./compoments/LayoutAside.vue";
import LayoutFooter from "./compoments/LayoutFooter.vue";
import LayoutContent from "./compoments/LayoutContent.vue";

export default defineComponent({
  name: 'LayoutContainer',
  components: {LayoutContent, LayoutFooter, LayoutAside, LayoutHeader, TinyContainer}
});
</script>
