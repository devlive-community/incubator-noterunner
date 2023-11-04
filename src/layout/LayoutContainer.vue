<template>
  <div class="demo-container coBox2">
    <TinyContainer pattern="simple"
                   :style="{backgroundColor: '#FFFFFF'}">
      <template #aside>
        <LayoutAside :height="windowHeight - headerHeight"
                     :width="asideWidth"
                     :node="selectedNode"
                     @onClick="handlerAsideClick($event)">
        </LayoutAside>
      </template>
      <LayoutContent :height="windowHeight"
                     :note="note"
                     @onClick="handlerContentClick">
      </LayoutContent>
    </TinyContainer>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue"
import {Container as TinyContainer} from '@opentiny/vue'
import LayoutAside from "./compoments/LayoutAside.vue"
import LayoutContent from "./compoments/LayoutContent.vue"
import {Note} from "../model/note.ts";

export default defineComponent({
  name: 'LayoutContainer',
  components: {LayoutContent, LayoutAside, TinyContainer},
  data() {
    return {
      windowWidth: document.documentElement.clientWidth,
      windowHeight: document.documentElement.clientHeight,
      headerHeight: 55,
      asideWidth: 200,
      note: undefined as Note | undefined,
      selectedNode: undefined as Note | undefined
    }
  },
  methods: {
    handlerAsideClick(value: Note) {
      this.note = value
    },
    handlerContentClick(value: Note) {
      this.selectedNode = value
    }
  }
});
</script>
