<template>
  <div class="demo-container coBox2">
    <TinyContainer pattern="simple"
                   :style="{backgroundColor: '#FFFFFF'}">
      <template #aside>
        <LayoutAside :height="windowHeight - headerHeight"
                     :width="asideWidth"
                     @onClick="handlerAsideClick($event)">
        </LayoutAside>
      </template>
      <LayoutContent :height="windowHeight"
                     :note="note">
      </LayoutContent>
    </TinyContainer>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue"
import {Container as TinyContainer} from '@opentiny/vue'
import LayoutHeader from "./compoments/LayoutHeader.vue"
import LayoutAside from "./compoments/LayoutAside.vue"
import LayoutFooter from "./compoments/LayoutFooter.vue"
import LayoutContent from "./compoments/LayoutContent.vue"
import {Note} from "../model/note.ts";

export default defineComponent({
  name: 'LayoutContainer',
  components: {LayoutContent, LayoutFooter, LayoutAside, LayoutHeader, TinyContainer},
  data() {
    return {
      windowWidth: document.documentElement.clientWidth,
      windowHeight: document.documentElement.clientHeight,
      headerHeight: 55,
      asideWidth: 200,
      note: null as Note
    }
  },
  methods: {
    handlerAsideClick(value: Note) {
      this.note = value
    }
  }
});
</script>
