MeleeUnit = {}

function MeleeUnit:new(o)
    o = o or {}
    o.range = 0.0
    o.max_hp = 255.0
    o.shape = "triangle"
    o.base_len = 10.0
    o.damage = { min = 10.0, max = 15.0 }

    setmetatable(o,self)
    self.__index = self
    return o
end

function MeleeUnit:on_attack(target)
end

function MeleeUnit:on_damage(assailant, damage_taken)
end

function MeleeUnit:on_death(killer)
end

function on_unit_death(dead, killer, world)
    if world:remaining_units(dead:player()) == 0 then
        world:victory()
    end    
end

function on_unit_damage(damaged, assailant, damage_done, world)
    if damaged:player() ~= assailant:player() then
        world:emit_reward{player=damaged:player(), reward=-damage_done, type="damage_taken"}
        world:emit_reward{player=assailant:player(), reward=damage_done, type="damage_dealt"}
    end
end

function on_victory(player, world)
    world:emit_reward{player=player, reward=100.0, type="victory"}
end

function on_defeat(player, world)
    world:emit_reward{player=player, reward=-100.0, type="defeat"}
end

-- function init_unit_types(world)
--     world:register(MeleeUnit)
-- end

function reset(builder)
    builder:set_players({
        1 = {r=0, g=255, b=0}
        2 = {r=255, g=0, b=0}
    })

    for i = 1,10 do
        y = 35.0
        if (i-1) // 5 == 1 then
            y = y + 20
        end

        builder:create_unit(MeleeUnit:new{pos={x = 10.0 * i + 1, y = y}, player= i // 5})
    end

    builder:set_player_color(0, {r=0,g=255,b=0})
    builder:set_player_color(1, {r=255,g=0,b=0})
end