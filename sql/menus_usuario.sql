select titulo, icone, link from menus where tipo_usuario = :tipo_usuario or :tipo_usuario = 'ADMIN'