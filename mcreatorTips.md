# creez une bare de vie comme les boss

import net.minecraft.server.level.ServerBossEvent;	
private final ServerBossEvent bossInfo = new ServerBossEvent(this.getDisplayName(), ServerBossEvent.BossBarColor.YELLOW, ServerBossEvent.BossBarOverlay.PROGRESS);
@Override
public void startSeenByPlayer(ServerPlayer player) {
	super.startSeenByPlayer(player);
	this.bossInfo.addPlayer(player);
}
@Override
public void stopSeenByPlayer(ServerPlayer player) {
	super.stopSeenByPlayer(player);
	this.bossInfo.removePlayer(player);
}
