<template>
  <div class="layout">
    <TinyContainer pattern="fashion"
                   :header-height="headerHeight">
      <template #header>
        <LayoutHeader :height="headerHeight">
        </LayoutHeader>
      </template>
      <template #aside>
        <LayoutAside :height="windowHeight">
        </LayoutAside>
      </template>
      <LayoutContent :height="windowHeight - headerHeight">
      </LayoutContent>
      <template #footer>
        <LayoutFooter></LayoutFooter>
      </template>
    </TinyContainer>
    <div style="margin-top: 50px;">
      Width: {{ windowWidth }}
      Height: {{ windowHeight }}
    </div>
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
      headerHeight: 30
    }
  },
  watch: {
    windowHeight(val) {
      let that = this
      console.log("实时屏幕高度：", val, that.windowHeight)
    },
    windowWidth(val) {
      let that = this
      console.log("实时屏幕宽度：", val, that.windowHeight)
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
