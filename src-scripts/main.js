import * as mc from '@minecraft/server';
import * as ui from '@minecraft/server-ui';

mc.system.run(() => {
    mc.world.sendMessage("This is working! Violet is pairing the scripts!")
})

mc.world.afterEvents.itemUse.subscribe(() => {
    mc.world.sendMessage("This is working! Violet is pairing the scripts!")
})