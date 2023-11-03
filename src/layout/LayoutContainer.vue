<template>
  <div class="layout">
    <TinyContainer pattern="fashion"
                   :style="{backgroundColor: '#FFFFFF'}"
                   :header-height="headerHeight"
                   :aside-width="asideWidth">
      <template #header>
        <LayoutHeader :height="headerHeight">
        </LayoutHeader>
      </template>
      <template #aside>
        <LayoutAside :height="windowHeight"
                     :width="asideWidth">
        </LayoutAside>
      </template>
      <LayoutContent :height="windowHeight - headerHeight">
      </LayoutContent>
      <template #footer>
        <LayoutFooter></LayoutFooter>
      </template>
    </TinyContainer>
  </div>
</template>

<script>
import {defineComponent} from "vue"
import {Container as TinyContainer} from '@opentiny/vue'
import LayoutHeader from "./compoments/LayoutHeader.vue"
import LayoutAside from "./compoments/LayoutAside.vue"
import LayoutFooter from "./compoments/LayoutFooter.vue"
import LayoutContent from "./compoments/LayoutContent.vue"

export default defineComponent({
  name: 'LayoutContainer',
  components: {LayoutContent, LayoutFooter, LayoutAside, LayoutHeader, TinyContainer},
  data() {
    return {
      windowWidth: document.documentElement.clientWidth,
      windowHeight: document.documentElement.clientHeight,
      headerHeight: 30,
      asideWidth: 200
    }
  },
  mounted() {
    // Hang the window.onresize event to the mounted function so that the component updates when its window changes
    const that = this
    window.onresize = () => {
      return (() => {
        window.fullHeight = document.documentElement.clientHeight
        window.fullWidth = document.documentElement.clientWidth
        that.windowHeight = window.fullHeight
        that.windowWidth = window.fullWidth
      })()
    }
  }
});
</script>
